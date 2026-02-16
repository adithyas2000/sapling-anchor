import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sapling } from "../target/types/sapling";
import { expect } from "chai";

describe("sapling", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.sapling as Program<Sapling>;

  it("should initialize", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({}).rpc();
    console.log("Your transaction signature", tx);
  });
  it("should add tree variant", async () => {
    const tx = await program.methods.addTreeVariant("1111", "Oak", new anchor.BN(10000), new anchor.BN(36)).accounts({}).rpc();
    console.log("Your transaction signature", tx);
    const [treeVariantPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("tree_variant")], program.programId);
    const treeVariantAccount = await program.account.treeVariantAccount.fetch(treeVariantPDA);
    console.log(treeVariantAccount.treeVariants);
    expect(treeVariantAccount.treeVariants.length).to.equal(1);
    expect(treeVariantAccount.treeVariants[0].treeTypeId).to.equal("1111");
    expect(treeVariantAccount.treeVariants[0].treeTypeName).to.equal("Oak");
    expect(treeVariantAccount.treeVariants[0].constPerMonth.toString()).to.equal(new anchor.BN(10000).toString());
    expect(treeVariantAccount.treeVariants[0].maxLifetimeInMonths.toString()).to.equal(new anchor.BN(36).toString());
  });
  it("should not add tree variant if not admin", async () => {
    const user = anchor.web3.Keypair.generate();
    await anchor.getProvider().connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL * 2);
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
  it("should remove tree variant", async () => {
    const tx = await program.methods.removeTreeVariant("1111").accounts({}).rpc();
    console.log("Your transaction signature", tx);
    const [treeVariantPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("tree_variant")], program.programId);
    const treeVariantAccount = await program.account.treeVariantAccount.fetch(treeVariantPDA);
    console.log(treeVariantAccount.treeVariants);
    expect(treeVariantAccount.treeVariants.length).to.equal(0);
  });
  it("should not remove tree variant if not admin", async () => {
    const user = anchor.web3.Keypair.generate();
    await anchor.getProvider().connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL * 2);
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
      expect(e.message).to.include("Tree variant not found");
    }
  });
});
