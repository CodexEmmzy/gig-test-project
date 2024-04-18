import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
describe("Test transfers", () => {

});
 it("transferSplTokens", async () => {
    // Generate keypairs for the new accounts
    const fromKp = pg.wallet.keypair;
    const toKp = new web3.Keypair();

    // Create a new mint and initialize it
    const mintKp = new web3.Keypair();
    const mint = await createMint(
      pg.program.provider.connection,
      pg.wallet.keypair,
      fromKp.publicKey,
      null,
      0
    );

    // Create associated token accounts for the new accounts
    const fromAta = await createAssociatedTokenAccount(
      pg.program.provider.connection,
      pg.wallet.keypair,
      mint,
      fromKp.publicKey
    );
    const toAta = await createAssociatedTokenAccount(
      pg.program.provider.connection,
      pg.wallet.keypair,
      mint,
      toKp.publicKey
    );
    // Mint tokens to the 'from' associated token account
    const mintAmount = 1000;
    await mintTo(
      pg.program.provider.connection,
      pg.wallet.keypair,
      mint,
      fromAta,
      pg.wallet.keypair.publicKey,
      mintAmount
    );

    // Send transaction
    const transferAmount = new BN(500);
    const txHash = await pg.program.methods
      .transferSplTokens(transferAmount)
      .accounts({
        from: fromKp.publicKey,
        fromAta: fromAta,
        toAta: toAta,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([pg.wallet.keypair, fromKp])
      .rpc();
    console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    await pg.connection.confirmTransaction(txHash, "finalized");
    const toTokenAccount = await pg.connection.getTokenAccountBalance(toAta);
    assert.strictEqual(
      toTokenAccount.value.uiAmount,
      transferAmount.toNumber(),
      "The 'to' token account should have the transferred tokens"
    );
  });