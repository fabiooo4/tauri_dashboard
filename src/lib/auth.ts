import { toast } from "svelte-sonner";
import { goto } from "$app/navigation";
import { userSchema } from "$lib/types/schemas";
import { invoke } from "@tauri-apps/api/core";

export async function logout() {
  await invoke("logout")
    .then(() => {
      toast.success("Logged out successfully");

      goto("/");
    })
    .catch((e) => console.error(e));
}

export async function getAuthenticatedUser(): Promise<User> {
  return await invoke("get_current_user").then((u) => {
    if (u !== null) return userSchema.parse(u);
    else throw "Not logged in";
  });
}
