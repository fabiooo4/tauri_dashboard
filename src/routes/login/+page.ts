import type { PageLoad } from "./$types.js";
import { superValidate } from "sveltekit-superforms";
import { userFormSchema } from "$lib/types/schemas.js";
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageLoad = async () => {
 return {
  form: await superValidate(zod(userFormSchema)),
 };
};
