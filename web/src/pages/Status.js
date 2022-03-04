import React, { useEffect, useState } from "react";
import axios from "axios";
import { apiUrl } from "../config";
import moment from "moment";

const Home = () => {
  let [submitList, setSubmitList] = useState([]);
  const request = async () => {
    await axios.get(`${apiUrl}/status`).then((res) => {
      setSubmitList(res.data);
    });
  };
  useEffect(() => request(), []);
  const time_convert = (time) => {
    let moment_time = moment(time);
    return moment_time.format("YYYY-MM-DD HH:mm:ss");
  };
  return (
    <div>
      <h1>Status</h1>
      <p>제출현황을 보여주는 페이지입니다.</p>

      <table className="table table-bordered table-striped">
        <thead>
          <tr>
            <th scope="col">제출번호</th>
            <th scope="col">문제번호</th>
            <th scope="col">유저(추후 수정)</th>
            <th scope="col">결과</th>
            <th scope="col">제출시간</th>
            <th scope="col">언어</th>
          </tr>
        </thead>
        <tbody>
          {submitList.map((submit) => (
            <tr key={submit.sid}>
              <th scope="row">{submit.sid}</th>
              <td>{submit.pid}</td>
              <td>{submit.uid}</td>
              <td style={{ color: submit.result === 0 ? "green" : "red" }}>
                {submit.result === 0 ? "맞았습니다!!!" : "틀렸습니다!!!"}
              </td>
              <td>{time_convert(submit.submit_at)}</td>
              <td>{submit.language}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Home;
