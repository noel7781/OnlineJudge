import { useParams } from "react-router-dom";
import axios from "axios";
import { apiUrl } from "../config";
import { useState, useEffect } from "react";

const Problem = () => {
  const { id } = useParams();
  const [data, setData] = useState("요청 전");
  const request = async (id) => {
    const res = await axios.get(`${apiUrl}/problem/${id}`);
    setData(res.data);
  };
  useEffect(() => request(id), [id]);
  return (
    <div>
      <h1 style={{ fontFamily: "NanumGothic" }}>
        {data.id}. {data.title}
      </h1>
      <div className="position-relative">
        <button
          onClick={() => (window.location.href = `/problem/${id}/submission`)}
          className="position-absolute top-0 end-0 col-2"
        >
          제출
        </button>
      </div>
      <br />
      <br />
      <table className="table ">
        <thead>
          <tr>
            <th scope="col">시간제한</th>
            <th scope="col">메모리제한</th>
            <th scope="col">제출수</th>
            <th scope="col">정답수</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>{data.time_limit}초</td>
            <td>{data.memory_limit}MB</td>
            <td>{data.submit_cnt}회</td>
            <td>{data.accepted_cnt}회</td>
          </tr>
        </tbody>
      </table>
      <br />
      <br />
      <br />
      <h3>문제</h3>
      <p>{data.description}</p>
      <hr />
      <br />
      <br />
      <h3>입력</h3>
      <p>{data.input_desc}</p>
      <hr />
      <br />
      <br />
      <h3>출력</h3>
      <p>{data.output_desc}</p>
      <hr />
      <br />
      <br />
    </div>
  );
};

export default Problem;
