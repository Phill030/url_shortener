import { useEffect } from "react";
import style from "./not-found.module.scss";
import IconButton, { IconPosition } from "../../components/button/icon-button.tsx";
import { useNavigate } from "react-router-dom";

export default function NotFound() {
  const navigate = useNavigate();

  useEffect(() => {
    document.title = "Page not found";
  }, []);

  return (
    <div className={style.wrapper}>
      <h1 className={style.text}>404</h1>
      <h2 className={style.text2}>Not found</h2>
      <IconButton
        text={"Return to home"}
        icon={"ion:arrow-back"}
        iconPosition={IconPosition.LEFT}
        className={style.returnButton}
        onClick={() => navigate(-1)}
      />
    </div>
  );
}
