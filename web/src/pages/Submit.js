import Highlight from "react-highlight";
import { useEffect, useState } from "react";

const supportedLang = ["c++", "c", "java", "python", "javascript"];

const Submit = (id) => {
  //   useEffect(() => hljs.highlightAll(), []);
  const onSubmit = (e) => {
    e.preventDefault();
    console.log(e);
    // let index = e.target.form.find((it) => it.checked === true);
    let language = "c++";

    console.log(e.target.form[0]);
    console.log(e.target.form[1].value);
    // let language = e.target.form.find((it) => it.checked === true);
    for (let i in supportedLang) {
      if (e.target.form[Number(i) + 1].checked === true) {
        language = supportedLang[Number(i)];
        break;
      }
    }
    const source_code = e.target.form[6].value;
    console.log("language:", language);
    console.log("source code:", source_code);
  };
  const [textArea, setTextArea] = useState("hi");
  const [lang, setLang] = useState("cpp");
  const onChange = (e) => {
    setTextArea(e.target.value);
  };
  return (
    <form id="code_form" onSubmit={onSubmit}>
      <button type="button" className="btn btn-success" onClick={onSubmit}>
        Submit
      </button>
      <div className="form-check">
        {supportedLang.map((lang, index) => {
          return (
            <div key={index + "_lang"}>
              <input
                className="form-check-input"
                type="radio"
                name="language"
                id={index}
                onClick={(e) => setLang(supportedLang[e.target.id])}
              />
              <label className="form-check-label">{lang}</label>
            </div>
          );
        })}
      </div>
      <Highlight language={lang}>{textArea}</Highlight>
      <textarea value={textArea.value} onChange={onChange} />
    </form>
  );
};

export default Submit;
