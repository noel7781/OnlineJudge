import Highlight from "react-highlight";
import { useEffect, useState } from "react";
import axios from "axios";
import { apiUrl } from "../config";
import { useParams } from "react-router-dom";
import WithLineNumbers from "../components/WIthLineNumbers";
import Editor from "../components/Editor";

const supportedLang = [
  "c++",
  "c",
  "java(not yet)",
  "python",
  "javascript(not yet)",
];

const Submit = () => {
  const { id } = useParams();

  const showResult = (result) => {
    if (result.data === "Success") {
      alert("정답입니다!!!");
    } else {
      alert("틀렸습니다!!!");
    }
  };
  const onSubmit = async (e) => {
    e.preventDefault();
    let language = "c++";

    // console.log(e.target.form[0]);
    // console.log(e.target.form[1].value);
    for (let i in supportedLang) {
      if (e.target.form[Number(i) + 1].checked === true) {
        language = supportedLang[Number(i)];
        break;
      }
    }
    const source_code = e.target.form[6].value;
    // console.log("language:", language);
    // console.log("source code:", source_code);
    // console.log("id: ", id);
    const res = await axios.post(`${apiUrl}/submit`, {
      problem_id: Number(id),
      source_code: source_code,
      language: language,
    });
    showResult(res);
    console.log("submit result:", res);
  };
  const [code, setCode] = useState("");
  const [lang, setLang] = useState("cpp");
  // const onChange = (e) => {
  //   setCode(e.target.value);
  // };
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
      {/* <Highlight language={lang}>{textArea}</Highlight>
      <textarea value={textArea.value} onChange={onChange} /> */}
      <Editor code={code} setCode={setCode} lang={lang} />
    </form>
  );
};

export default Submit;
