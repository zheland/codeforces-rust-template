/*
about:debugging#/runtime/this-firefox
*/
(() => {
  let menu_list = document.querySelector(".second-level-menu-list");
  let sample_test = document.querySelector(".sample-test");
  let sample_tests = document.querySelector(".sample-tests");
  let sample_test_title = sample_tests && sample_tests.querySelector(".section-title");
  if (menu_list === null || sample_test === null || sample_tests === null || sample_test_title === null) {
    return;
  }

  let previous1 = document.getElementById("copy-examples-1");
  if (previous1 !== null) {
    previous1.parentNode.removeChild(previous1);
  }
  let previous2 = document.getElementById("copy-examples-2");
  if (previous2 !== null) {
    previous2.parentNode.removeChild(previous2);
  }
  let a1 = document.createElement("a");
  a1.appendChild(document.createTextNode("СКОПИРОВАТЬ ПРИМЕРЫ"));
  a1.href = "javascript:void(0);";
  let li = document.createElement("li");
  li.id = "copy-examples-1";
  li.appendChild(a1);
  menu_list.appendChild(li);

  let a2 = document.createElement("div");
  a2.id = "copy-examples-2";
  a2.classList.add("input-output-copier");
  a2.appendChild(document.createTextNode("Скопировать все"));
  a2.href = "javascript:void(0);";
  sample_test_title.appendChild(a2);

  let handler = () => {
    let children = [...sample_test.childNodes].map((node) =>
      node
        .querySelector("pre")
        .innerText
        .replace(/\n*$/, "")
      /*
      node
        .querySelector("pre")
        .innerHTML.split(/<br>|\n/)
        .map((text) => text.trim())
        .join("\n")
        .replace(/\n$/, "")
      */
    );
    let seperator1 = "--";
    let seperator2 = "==";
    for (let child of children) {
      while (
        child.indexOf(seperator1) != -1 ||
        child.indexOf(seperator2) != -1
      ) {
        seperator1 += "-";
        seperator2 += "=";
      }
    }
    let text = seperator1 + "\n" + seperator2 + "\n";
    for (let child_idx in children) {
      let child = children[child_idx];
      if (child_idx > 0 && (child_idx & 1) == 1) {
        text += "\n" + seperator1 + "\n";
      } else if (child_idx > 0 && (child_idx & 1) == 0) {
        text += "\n" + seperator2 + "\n";
      }
      text += child;
    }
    navigator.clipboard.writeText(text);
  };

  li.addEventListener("click", handler);
  a2.addEventListener("click", handler);
})();
