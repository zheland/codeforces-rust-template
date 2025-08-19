/*
  https://developer.mozilla.org/ru/docs/Mozilla/Add-ons/WebExtensions/Your_first_WebExtension
  about:debugging#/runtime/this-firefox
*/

(() => {
  let menu_list = document.querySelector(".second-level-menu-list");
  let sample_test = document.querySelector(".sample-test");
  let sample_tests = document.querySelector(".sample-tests");
  let sample_test_title =
    sample_tests && sample_tests.querySelector(".section-title");
  if (
    menu_list === null ||
    sample_test === null ||
    sample_tests === null ||
    sample_test_title === null
  ) {
    return;
  }

  for (let id of ["copy-examples-1", "copy-examples-2"]) {
    let previous = document.getElementById(id);
    if (previous !== null) {
      previous.parentNode.removeChild(previous);
    }
  }

  let a1 = document.createElement("a");
  a1.appendChild(document.createTextNode("СКОПИРОВАТЬ ПРИМЕРЫ ДЛИННО"));
  a1.href = "javascript:void(0);";
  let li1 = document.createElement("li");
  li1.id = "copy-examples-1";
  li1.appendChild(a1);
  menu_list.appendChild(li1);

  let a2 = document.createElement("a");
  a2.appendChild(document.createTextNode("СКОПИРОВАТЬ ПРИМЕРЫ СЖАТО"));
  a2.href = "javascript:void(0);";
  let li2 = document.createElement("li");
  li2.id = "copy-examples-1";
  li2.appendChild(a2);
  menu_list.appendChild(li2);

  let a3 = document.createElement("div");
  a3.id = "copy-examples-2";
  a3.classList.add("input-output-copier");
  a3.appendChild(document.createTextNode("Скопировать все длинно"));
  a3.href = "javascript:void(0);";
  sample_test_title.appendChild(a3);

  let a4 = document.createElement("div");
  a4.id = "copy-examples-2";
  a4.classList.add("input-output-copier");
  a4.appendChild(document.createTextNode("Скопировать все сжато"));
  a4.href = "javascript:void(0);";
  sample_test_title.appendChild(a4);

  let handler = (is_short) => {
    let children = [...sample_test.childNodes].map((node) =>
      node.querySelector("pre").innerText.replace(/\n*$/, "")
    );
    let test_separator = "==";
    let io_separator = "--";
    let separator3;
    if (is_short) {
      separator3 = " ";
    } else {
      separator3 = "\n";
    }
    for (let child of children) {
      while (
        child.indexOf(test_separator) != -1 ||
        child.indexOf(io_separator) != -1
      ) {
        test_separator += "=";
        io_separator += "-";
      }
    }
    let text = test_separator + "\n" + io_separator;
    for (let child_idx in children) {
      if ((child_idx & 1) == 0) {
        text += "\n" + test_separator + separator3;
      } else if ((child_idx & 1) == 1) {
        text += separator3 + io_separator + separator3;
      }

      let child = children[child_idx];
      if (is_short) {
        child = child.replaceAll(/\n/g, "  ");
      }
      text += child;
    }
    navigator.clipboard.writeText(text);
  };

  li1.addEventListener("click", () => handler(false));
  li2.addEventListener("click", () => handler(true));
  a3.addEventListener("click", () => handler(false));
  a4.addEventListener("click", () => handler(true));
})();
