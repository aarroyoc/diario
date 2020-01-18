window.addEventListener("load",()=>{
    let editor = document.getElementById("editor");
    let output = document.getElementById("output");
    output.innerHTML = editor.value;
    editor.addEventListener("input",()=>{
        output.innerHTML = editor.value;
    });
});