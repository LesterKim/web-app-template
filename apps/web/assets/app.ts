document.addEventListener("htmx:afterSwap", (event) => {
  const target = event.target as HTMLElement | null;
  if (target) {
    target.classList.add("htmx-swapped");
  }
});
