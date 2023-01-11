import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Liquifier } from "../target/types/liquifier";

describe("liquifier", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Liquifier as Program<Liquifier>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
