import { Icon } from "@iconify/react/dist/iconify.js";
import style from "./icon-input.module.scss";

export interface IconInput {
  icon: string;
  iconSize?: string;
  className?: string;
  type?: string;
  value?: string;
  onChange?: (event: React.ChangeEvent<HTMLInputElement>) => void;
  required?: boolean;
  placeholder?: string;
  pattern?: string;
}

export default function IconInput({
  icon,
  iconSize = "25px",
  type = "text",
  className,
  value,
  onChange,
  required = false,
  placeholder = "",
  pattern = undefined,
}: IconInput) {
  return (
    <div className={`${style.input} ${className}`}>
      <Icon icon={icon} width={iconSize} />
      <input type={type} value={value} onChange={onChange} required={required} placeholder={placeholder} pattern={pattern} />
    </div>
  );
}
