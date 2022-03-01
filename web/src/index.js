import "bootstrap/dist/css/bootstrap.min.css";
import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import MenuBar from "./MenuBar";
import reportWebVitals from "./reportWebVitals";
import { BrowserRouter, Routes, Route } from "react-router-dom";
// page component
import Home from "./pages/Home";
import Problems from "./pages/Problems";
import Status from "./pages/Status";
import Discuss from "./pages/Discuss";
import NotFound from "./pages/NotFound";
import Problem from "./pages/Problem";
import Submit from "./pages/Submit";

ReactDOM.render(
  <>
    <MenuBar />
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/problems" element={<Problems />} />
        <Route path="/problem/:id" element={<Problem />} />
        <Route path="/problem/:id/submission" element={<Submit />} />
        <Route path="/status" element={<Status />} />
        <Route path="/discuss" element={<Discuss />} />
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  </>,
  document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
