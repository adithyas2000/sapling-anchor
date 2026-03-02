import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sapling } from "../target/types/sapling";
import { expect } from "chai";
import { getFundsToWallet } from "./utils";
import { TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddressSync, getMint } from "@solana/spl-token";

describe("sapling", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.sapling as Program<Sapling>;
  const userWallet = anchor.web3.Keypair.generate();
  console.log("User wallet public key", userWallet.publicKey.toBase58());

  it("should initialize", async () => {

    const tx = await program.methods.initialize().accountsPartial({ deployer: anchor.getProvider().wallet.publicKey }).rpc();
    console.log("Your transaction signature", tx);
  });
  it("should add tree variant", async () => {
    const tx = await program.methods.addTreeVariant("1111", "Oak", new anchor.BN(10000), new anchor.BN(36)).accounts({}).rpc();
    console.log("Your transaction signature", tx);
    const [treeVariantPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("tree_variant"), Buffer.from("1111")], program.programId);
    const treeVariantAccount = await program.account.treeVariant.fetch(treeVariantPDA);
    expect(treeVariantAccount.treeTypeId).to.equal("1111");
    expect(treeVariantAccount.treeTypeName).to.equal("Oak");
    expect(treeVariantAccount.costPerMonth.toString()).to.equal(new anchor.BN(10000).toString());
    expect(treeVariantAccount.maxLifetimeInMonths.toString()).to.equal(new anchor.BN(36).toString());
  });


  // it("should init user", async () => {

  //   const tx = await program.methods.initUser().accountsPartial({ signer: userWallet.publicKey }).signers([userWallet]).rpc();
  //   console.log("Your transaction signature", tx);
  // });
  it("should rent tree", async () => {
    await getFundsToWallet(anchor.getProvider().connection, userWallet.publicKey, 5);
    const rentalId = "1111";
    const rentDurationMonths = new anchor.BN(12);

    const [mintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("token_mint"), userWallet.publicKey.toBuffer(), Buffer.from(rentalId)],
      program.programId
    );
    const [treeMetadataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tree_metadata"), userWallet.publicKey.toBuffer(), Buffer.from(rentalId)],
      program.programId
    );
    console.log("Mint PDA", mintPDA.toBase58());
    const tx = await program.methods.rentTree(rentalId, rentDurationMonths).accountsPartial({
      signer: userWallet.publicKey,
      mint: mintPDA,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    }).signers([userWallet]).rpc();
    const treeMetadataAccount = await program.account.treeMetadata.fetch(treeMetadataPDA);
    expect(treeMetadataAccount.condition).to.equal("OK");
    expect(treeMetadataAccount.mint.toBase58()).to.equal(mintPDA.toBase58());
    expect(treeMetadataAccount.level).to.equal(1);
    expect(treeMetadataAccount.owner.toBase58()).to.equal(userWallet.publicKey.toBase58());
    expect(treeMetadataAccount.treeId).to.equal(rentalId);
    expect(treeMetadataAccount.remainingMonths.toString()).to.equal(rentDurationMonths.toString());
    console.log("User account", userWallet.publicKey.toBase58());
    console.log("Your transaction signature", tx);
  });


  it("should not add tree variant if not admin", async () => {
    const user = anchor.web3.Keypair.generate();
    await getFundsToWallet(anchor.getProvider().connection, user.publicKey, 5);
    const failingTx = async () => {
      const tx = await program.methods
        .addTreeVariant("2222", "Maple", new anchor.BN(20000), new anchor.BN(36))
        .accounts({ caller: user.publicKey, admin: user.publicKey })
        .signers([user])
        .rpc();
    }
    try {
      await failingTx();
      expect.fail("Expected transaction to fail");
    } catch (e) {
      expect(e.message).to.include("Caller is not the admin");
    }

  });
  it("should not remove tree variant if not admin", async () => {
    const user = anchor.web3.Keypair.generate();
    await getFundsToWallet(anchor.getProvider().connection, user.publicKey, 2);
    const failingTx = async () => {
      const tx = await program.methods.removeTreeVariant("1111")
        .accounts({ caller: user.publicKey, admin: user.publicKey })
        .signers([user]).rpc();
    }
    try {
      await failingTx();
      expect.fail("Expected transaction to fail");
    } catch (e) {
      expect(e.message).to.include("Caller is not the admin");
    }
  });
  it("should not remove tree variant if not found", async () => {
    const failingTx = async () => { const tx = await program.methods.removeTreeVariant("3333").accounts({}).rpc(); }
    try {
      await failingTx();
      expect.fail("Expected transaction to fail");
    } catch (e) {
      expect(e.message).to.include("The program expected this account to be already initialized");
    }
  });
  it("should remove tree variant", async () => {
    const tx = await program.methods.removeTreeVariant("1111").accounts({}).rpc();
    console.log("Your transaction signature", tx);
    const [treeVariantPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("tree_variant"), Buffer.from("1111")], program.programId);
    const treeVariantAccount = await anchor.getProvider().connection.getAccountInfo(treeVariantPDA);
    expect(treeVariantAccount).to.be.null;
  });

});
