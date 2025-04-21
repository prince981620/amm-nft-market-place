import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftMarketplace } from "../target/types/nft_marketplace";
import { Keypair } from "@solana/web3.js";

describe("nft-marketplace", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();

  const connection = provider.connection;

  const [admin, maker, taker, makermint] = Array.from({ length: 4 }, () =>
    Keypair.generate()
  );
  const program = anchor.workspace.NftMarketPlace as Program<NftMarketplace>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(
      "solavatar",
      60,
    )
    .accounts({

    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
