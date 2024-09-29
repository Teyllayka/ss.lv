// svelte.d.ts
declare namespace svelte.JSX {
  interface HTMLAttributes<T> {
    onClickOutside?: (event: CustomEvent<any>) => void;
  }
}
