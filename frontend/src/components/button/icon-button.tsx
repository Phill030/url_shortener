import { Icon } from "@iconify/react/dist/iconify.js";
import style from "./icon-button.module.scss";

export interface IconButton {
  text: string;
  icon: string;
  iconPosition?: IconPosition;
  iconSize?: string;
  className?: string;
  onClick: () => void;
  disabled?: boolean;
}

export enum IconPosition {
  LEFT,
  RIGHT,
}

export default function IconButton({ text, icon, iconPosition, iconSize = "25px", className, onClick, disabled = false }: IconButton) {
  return (
    <button className={`${style.button} ${className}`} onClick={onClick} disabled={disabled}>
      {iconPosition === IconPosition.RIGHT ? (
        <>
          <p>{text}</p> <Icon icon={icon} width={iconSize}/>
        </>
      ) : (
        <>
          <Icon icon={icon} width={iconSize}/> <p>{text}</p>
        </>
      )}
    </button>
  );
}
