import type { AvailableLanguageTag } from "../../lib/paraglide/runtime";
import type { ParaglideLocals } from "@inlang/paraglide-sveltekit";
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			paraglide: ParaglideLocals<AvailableLanguageTag>;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	declare namespace svelteHTML {
		interface HTMLAttributes<T> {
			onclick_outside?: CompositionEventHandler<T>;
		}
	}
}

export {};
