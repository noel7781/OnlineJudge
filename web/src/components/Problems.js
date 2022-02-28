import React, { useState, useRef } from "react";

const Problems = () => {
  const [problemList, setProblemList] = useState([
    { id: 1, title: "A + B", accuracy: 95, difficulty: "Silver" },
    { id: 2, title: "A - B", accuracy: 50, difficulty: "Gold" },
    { id: 3, title: "A * B", accuracy: 20, difficulty: "Platinum" },
  ]);
  const pageRef = useRef(1);

  return (
    <div>
      <h1>Problems</h1>
      <p>문제 목록을 보여주는 페이지입니다.</p>

      <table class="table table-borderless">
        <thead>
          <tr>
            <th scope="col">#</th>
            <th scope="col">Problem</th>
            <th scope="col">Accuracy</th>
            <th scope="col">Difficulty</th>
          </tr>
        </thead>
        <tbody>
          {problemList.map((problem) => (
            <tr
              key={problem.id}
              onClick={() => {
                window.location.href = `/problem/${problem.id}`;
              }}
            >
              <th scope="row">{problem.id}</th>
              <td>{problem.title}</td>
              <td>{problem.accuracy}%</td>
              <td>{problem.difficulty}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Problems;
