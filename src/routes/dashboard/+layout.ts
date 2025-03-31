import { getAuthenticatedUser } from "$lib/auth";
import { redirect } from "@sveltejs/kit";

export async function load() {
  let user: User = await getAuthenticatedUser().catch((e) => {
    console.error(e);
    redirect(308, "/");
  });

  return { user };
}
