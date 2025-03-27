import type { PageServerLoad } from "./$types.js";
import { superValidate } from "sveltekit-superforms";
import { userSchema } from "$lib/types/schemas.js";
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageServerLoad = async () => {
 return {
  form: await superValidate(zod(userSchema)),
 };
};
