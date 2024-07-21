import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolToken } from "../target/types/sol_token";

describe("sol-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolToken as Program<SolToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
