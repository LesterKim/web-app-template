document.addEventListener("htmx:afterSwap", function (event) {
  var target = event.target;
  if (target && target.classList) {
    target.classList.add("htmx-swapped");
  }
});
