import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import NotFound from "./NotFound.tsx";
import { BrowserRouter, Routes, Route } from "react-router";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<App />} />
      <Route path="/404" element={<NotFound />} />
    </Routes>
  </BrowserRouter>,
);
