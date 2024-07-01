import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ecom } from "../target/types/ecom";
import { PublicKey } from "@solana/web3.js";

const image = 'https://firebasestorage.googleapis.com/v0/b/solana-ecom.appspot.com/o/S12-R24D27_NECKTIESATINDRESS_IVORY-24218-BearePark-0793.webp?alt=media&token=fee0100e-76ff-424a-8789-b1129d66d0b8',


describe("ecom", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.Ecom as Program<Ecom>;

  const wallet = provider.wallet as anchor.Wallet;
  // const connection = provider.connection;

  const nonAdmin = anchor.web3.Keypair.generate();

  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("ecom4")],
    program.programId
  );

  // JUST NEED TO INITIALIZE ONCE!

  it("Is initialized!", async () => {
    // Add your test here.

    const txSig = await program.methods.initialize().accounts({
      products: counterPDA,
      user: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([wallet.payer]).rpc();

    console.log(`Transaction Signature: ${txSig}`);

  });

  it("Adds a product", async () => {
    const tx = await program.methods.addProduct( image, 0.1 ).accounts({ products: counterPDA, user: wallet.publicKey }).rpc();
    console.log('transaction: ', tx);
  });

  it("Is PURCHASES", async () => {
    
    const buyerAccount = provider.wallet.publicKey;
    const productIndex = 0; // The index of the product to purchase
    
    const transactionSignature = await program.methods.purchaseProduct(
      new anchor.BN(productIndex)
    )
    .accounts({
      buyer: buyerAccount,
      products: counterPDA,
      to: wallet.publicKey, // self owner
      // to: nonAdmin.publicKey, // test: pay to some thats not owner of the product 
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    console.log('sigature', transactionSignature);
    
    
  });


  it("List a product", async () => {
    const productIndex = 0;
    const transactionSignature = await program.methods.listProduct(
      new anchor.BN(productIndex)
    ).accounts({
      products: counterPDA,
      user: wallet.publicKey
    })
    // .signers([nonAdmin])
    .rpc();
    console.log(`Transaction Signature of listing a product: ${transactionSignature}`);
  });


  it("De-List a product", async () => {
    const productIndex = 0;
    const transactionSignature = await program.methods.delistProduct(
      new anchor.BN(productIndex)
    ).accounts({
      products: counterPDA,
      user: wallet.publicKey
    }).rpc();
    console.log(`Transaction Signature of delisting a product: ${transactionSignature}`);
  });

  it("Change price", async () => {
    const productIndex = 0;
    const newPrice = 0.5
    const transactionSignature = await program.methods.updateProductPrice(
      new anchor.BN(productIndex),
      newPrice
    ).accounts({
      products: counterPDA,
      user: wallet.publicKey
    })
    // .signers([nonAdmin]) // test: a non owner tries to change the price
    .rpc();
    console.log(`Transaction Signature: ${transactionSignature}`);
  });

  it("Is fetches products!", async () => {
    const accountData = await program.account.allProducts.fetch(counterPDA);
    console.log(accountData);
  });



});
