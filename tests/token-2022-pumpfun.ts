import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token2022Pumpfun } from "../target/types/token_2022_pumpfun";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { getAssociatedTokenAddress, getAssociatedTokenAddressSync, NATIVE_MINT, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { AddressLookupTableProgram, ComputeBudgetProgram, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { BN } from "bn.js";
import { getNftMetadataAddress, getOrcleAccountAddress, getPersonalPositionAddress, getPoolAddress, getPoolVaultAddress, getProtocolPositionAddress, getTickArrayAddress, getTickArrayBitmapAddress, i32ToBytes, initialize, openPosition, setupInitializeTest, waitFor } from "./utils";
import { DEVNET_PROGRAM_ID, SqrtPriceMath, TickUtils } from "@raydium-io/raydium-sdk-v2";
import { ClmmProgram, devConfigs } from "./config";
import { createAndSendV0Tx } from "./utils/transaction";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());
// const connection = new Connection("http://localhost:8899")
export const connection = new Connection("https://devnet.helius-rpc.com/?api-key=e2cc6225-fae1-4f90-a6b1-5684f49dec62", { commitment: "finalized" })

const payer = Keypair.fromSecretKey(bs58.decode("5BrUQk416xSy4xbHZq6jXb2JcVA8iRnPNJJr3NZv2wukMhwB39ndpe9eaCXmuFLxzkVUYXbdCB9ydeJkhKCGhnkm"))
const feeAccount = new PublicKey("3MQVpAwsccXHG7k6RvhwBVRCs3tfmHRW8VUYJUdyPBXd")
const program = anchor.workspace.Token2022Pumpfun as Program<Token2022Pumpfun>;
let mintAddr: Keypair;

describe("token-2022-pumpfun", () => {
  it("Is initialized!", async () => {

    const initializeArgu = {
      bondingCurveLimitation: new BN(85 * LAMPORTS_PER_SOL),
      initialVirtualSol: new BN(40 * LAMPORTS_PER_SOL),
      initialVirtualToken: new BN(1030000000).mul(new BN(LAMPORTS_PER_SOL)),
      createPoolFeeLamports: new BN(0.05 * LAMPORTS_PER_SOL),
      swapFee: 2.0,
    }

    const tx = await program.methods
      .initialize(initializeArgu)
      .accounts({ feeAccount: feeAccount })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
  });

  it("create", async () => {
    mintAddr = Keypair.generate()
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    console.log("mintAddr : ", mintAddr.publicKey.toBase58());

    const tx = await program.methods
      .create({
        name: "wiz05.06",
        symbol: "wizSym",
        uri: "wizUri",
        transferFeeBasisPoints: 50, ///   0.005 %
        maximumFee: new BN(5000)
      })     //   create Pool Fee 0.01 sol
      .accounts({
        mintAddr: mintAddr.publicKey,
        tokenPool: tokenPool,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([payer, mintAddr])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer, mintAddr]);
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`)
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
  });

  it("buy", async () => {

    const buyQuote = await getBuyQuote(3 * 10 ** 9, 2)

    const tx = await program.methods
      .buy(new BN(3 * 10 ** 9), buyQuote)     //   buy 0.1 sol
      .accounts({
        mintAddr: mintAddr.publicKey,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`)
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
  });

  it("sell", async () => {
    const sellQuote = await getSellQuote(10 ** 13, 2)

    // Add your test here.
    const tx = await program.methods
      .sell(new BN(10 ** 13), sellQuote)    //   buy amount / expected amount / slippage
      .accounts({
        mintAddr: mintAddr.publicKey,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`)
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
  });

  it("raydium integrate", async () => {
    const sqrtPriceX64 = 0
    const openTime = 0

    const tickLowerIndex = -10
    const tickUpperIndex = 10
    const liquidity = new BN(1_000_000_000);
    const amount0Max = new BN(2_999_548_675);
    const amount1Max = new BN('922759999999995112');
    const ammConfig = devConfigs[0]

    const opAddress = Keypair.generate()
    const positionNftMint = Keypair.generate();

    console.log("opAddress.publicKey : ", opAddress.publicKey);

    const [poolAddress, _bump1] = await getPoolAddress(
      new PublicKey(ammConfig.id),
      NATIVE_MINT,
      mintAddr.publicKey,
      DEVNET_PROGRAM_ID.CLMM
    );
    const [vault0, _bump2] = await getPoolVaultAddress(
      poolAddress,
      NATIVE_MINT,
      DEVNET_PROGRAM_ID.CLMM
    );
    const [vault1, _bump3] = await getPoolVaultAddress(
      poolAddress,
      mintAddr.publicKey,
      DEVNET_PROGRAM_ID.CLMM
    );
    const [observation, _bump5] = await getOrcleAccountAddress(
      poolAddress,
      DEVNET_PROGRAM_ID.CLMM
    );

    const tickArrayLowerStartIndex = TickUtils.getTickArrayStartIndexByTick(
      tickLowerIndex,
      ammConfig.tickSpacing
    );
    const [tickArrayLower] = await getTickArrayAddress(
      poolAddress,
      ClmmProgram,
      tickArrayLowerStartIndex
    );
    const tickArrayUpperStartIndex = TickUtils.getTickArrayStartIndexByTick(
      tickUpperIndex,
      ammConfig.tickSpacing
    );
    const [tickArrayUpper] = await getTickArrayAddress(
      poolAddress,
      ClmmProgram,
      tickArrayUpperStartIndex
    );

    const positionANftAccount = getAssociatedTokenAddressSync(
      positionNftMint.publicKey,
      opAddress.publicKey
    );

    const metadataAccount = (
      await getNftMetadataAddress(positionNftMint.publicKey)
    )[0];

    const [personalPosition] = await getPersonalPositionAddress(
      positionNftMint.publicKey,
      ClmmProgram
    );

    const [protocolPosition] = await getProtocolPositionAddress(
      poolAddress,
      ClmmProgram,
      tickLowerIndex,
      tickUpperIndex
    );

    const token0Account = getAssociatedTokenAddressSync(
      new PublicKey(NATIVE_MINT),
      opAddress.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    const token1Account = getAssociatedTokenAddressSync(
      new PublicKey(mintAddr.publicKey),
      opAddress.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );

    const [lookupTableInst, lookupTableAddress] =
      AddressLookupTableProgram.createLookupTable({
        authority: payer.publicKey,
        payer: payer.publicKey,
        recentSlot: await connection.getSlot() - 1,
      });

    const addAddressesInstruction = AddressLookupTableProgram.extendLookupTable({
      payer: payer.publicKey,
      authority: payer.publicKey,
      lookupTable: lookupTableAddress,
      addresses: [
        new PublicKey(ammConfig.id),
        NATIVE_MINT,
        TOKEN_2022_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        metadataAccount,
        mintAddr.publicKey,
        observation,
        opAddress.publicKey,
        personalPosition,
        positionANftAccount,
        positionNftMint.publicKey,
        protocolPosition,
        poolAddress,
        tickArrayLower,
        tickArrayUpper,
        token0Account,
        token1Account,
        vault0,
        vault1,
        payer.publicKey
      ]
    });

    console.log("waiting for confirming of creating LUT");
    await createAndSendV0Tx(connection, payer, [lookupTableInst, addAddressesInstruction])
    await waitFor(15000)

    const ix = await program.methods
      .proxyInitialize(SqrtPriceMath.getSqrtPriceX64FromTick(sqrtPriceX64), new BN(openTime))
      .accounts({
        observationState: observation,
        opAddress: opAddress.publicKey,
        ammConfig: new PublicKey(ammConfig.id),
        tokenMint0: NATIVE_MINT,
        tokenMint1: mintAddr.publicKey,
        tokenProgram0: TOKEN_PROGRAM_ID,
        tokenProgram1: TOKEN_2022_PROGRAM_ID,
      })
      .signers([payer])
      .instruction();

    const openIx = await program.methods
      .proxyOpenPosition(
        tickLowerIndex,
        tickUpperIndex,
        tickArrayLowerStartIndex,
        tickArrayUpperStartIndex,
        liquidity,
        amount0Max,
        amount1Max,
        true,
        true
      )
      .accounts({
        payer: opAddress.publicKey,
        positionNftOwner: opAddress.publicKey,
        positionNftMint: positionNftMint.publicKey,
        positionNftAccount: positionANftAccount,
        metadataAccount,
        protocolPosition,
        tickArrayLower,
        tickArrayUpper,
        personalPosition,
        poolState: poolAddress,
        tokenAccount0: token0Account,
        tokenAccount1: token1Account,
        tokenVault0: vault0,
        tokenVault1: vault1,
        vault0Mint: NATIVE_MINT,
        vault1Mint: mintAddr.publicKey,
      })
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 })
      ])
      .signers([positionNftMint])
      .instruction();

    const { value: lookupTable } = await connection.getAddressLookupTable(lookupTableAddress, { commitment: "finalized" });

    console.log("lookupTable : ", lookupTable);

    const openIxmessageV0 = new TransactionMessage({
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
      instructions: [
        SystemProgram.transfer({
          fromPubkey: payer.publicKey,
          toPubkey: opAddress.publicKey,
          lamports: 240_000_000
        }),
        ix,
        openIx,
      ]
    }).compileToV0Message([lookupTable]);

    const transaction = new VersionedTransaction(openIxmessageV0);

    // sign your transaction with the required `Signers`
    transaction.sign([opAddress, payer, positionNftMint]);

    const serialized = transaction.serialize()

    const size = serialized.length + 1 + (transaction.signatures.length * 64);

    console.log("size =======================> ", size);

    (await connection.simulateTransaction(transaction)).value.logs.forEach(ele => console.error(`${ele}`));

    const txId = await connection.sendTransaction(transaction);
    console.log(`https://solscan.io/tx/${txId}?cluster=devnet`);
    console.log(`https://solana.fm/tx/${txId}?cluster=devnet-solana`)
  })
});

const getBuyQuote = async (lamport, slippage) => {
  const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
  const { initVirtualSol, solReserves, initVirtualToken, kParam } = await program.account.bondingCurve.fetch(bondingCurve)

  const initVirtualSolNew = Number(initVirtualSol)
  const solReservesNew = Number(solReserves)
  const initVirtualTokenNew = Number(initVirtualToken)
  const tokenNew = initVirtualTokenNew - (Number(kParam) / (initVirtualSolNew + solReservesNew))

  let price = (initVirtualSolNew + solReservesNew) / (initVirtualTokenNew - tokenNew)

  return new BN(`${((1 - 0.01 * slippage) * lamport) / price}`)
}

const getSellQuote = async (token_amount, slippage) => {
  const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
  const { initVirtualSol, solReserves, initVirtualToken, kParam } = await program.account.bondingCurve.fetch(bondingCurve)

  const initVirtualSolNew = Number(initVirtualSol)
  const solReservesNew = Number(solReserves)
  const initVirtualTokenNew = Number(initVirtualToken)
  const tokenNew = initVirtualTokenNew - (Number(kParam) / (initVirtualSolNew + solReservesNew))

  let price = (initVirtualSolNew + solReservesNew) / (initVirtualTokenNew - tokenNew)

  return new BN(`${Math.floor((1 - 0.01 * slippage) * token_amount * price)}`)
}