import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftMarketplace } from "../target/types/nft_marketplace";
import { Commitment, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram, TOKEN_PROGRAM_ID as tokenProgram, createMint, createAccount, mintTo, getAssociatedTokenAddress, TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token"
import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"
describe("nft-marketplace", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const commitment: Commitment = "confirmed";

  const provider = anchor.getProvider();
  
  const program = anchor.workspace.nft_marketplace as Program<NftMarketplace>;
  
  const connection = provider.connection;

  const name = "avatar";

  const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  )
}

  const confirmTxs = async (signatures: string[]) => {
      await Promise.all(signatures.map(confirmTx))
  }


  const [admin, maker, taker] = Array.from({ length: 3 }, () =>
    Keypair.generate()
  );

  let nftMint: PublicKey;
  let nftMetadataPda: PublicKey;
  let nftEditionPda: PublicKey;
  let makerNftAta: PublicKey;

  console.log("admin :", admin.publicKey.toBase58());
  // seeds = [b"marketplace", name.as_str().as_bytes()],
  const marketplace = PublicKey.findProgramAddressSync(
    [Buffer.from("marketplace"), Buffer.from(name)],
    program.programId
  )[0];

  console.log("marketplace", marketplace);

  const treasury = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), marketplace.toBuffer()],
    program.programId
  )[0];

  console.log("treasury", treasury);


  const reward_mint = PublicKey.findProgramAddressSync(
    [Buffer.from("rewards"), marketplace.toBuffer()],
    program.programId
  )[0];

  console.log("reward_mint", reward_mint);

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${provider.connection.rpcEndpoint}`
    );
    return signature;
  };


  function findMetadataPda(mint: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        Buffer.from(MPL_TOKEN_METADATA_PROGRAM_ID.toString()),
        mint.toBuffer(),
      ],
      new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID)
    )[0];
  }


  function findMasterEditionPda(mint: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        Buffer.from(MPL_TOKEN_METADATA_PROGRAM_ID.toString()),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID)
    )[0];
  }


  it("airdrop and create Mints", async ()=>{
    await Promise.all([admin, maker, taker].map(async (k) => {
      return await anchor.getProvider().connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL)
    })).then(confirmTxs);


    [admin, maker, taker].map(async (key)=>{
      const balace = await connection.getBalance(key.publicKey);
      console.log("Balance : ",balace/LAMPORTS_PER_SOL);
    });

  })

  it("Is initialized!", async () => {
    console.log(
      "admin :", admin.publicKey,
      "marketplace :",marketplace,
      "treasury :",treasury,
      "rewardMint :",reward_mint,
      "tokenProgram :",tokenProgram,
      "systemProgram :", SystemProgram.programId
    )
    const tx = await program.methods.initialize(
      "avatar",
      60,
    )
    .accountsStrict({
      admin: admin.publicKey,
      marketplace,
      treasury,
      rewardMint:reward_mint,
      tokenProgram,
      systemProgram: SystemProgram.programId
    })
    .signers([admin])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("list NFT",async()=>{
  //   const tx = await program.methods.list(
  //     new anchor.BN(2),
  //   )
  //   .accountsStrict({
  //     maker: maker.publicKey,
  //     marketplace,

  //   })
  // })

});
