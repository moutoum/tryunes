import React, {ButtonHTMLAttributes} from 'react';
import classes from './Button.module.css';

interface ButtonProps {
    label: string,
    icon?: string,
    position?: 'before' | 'after',
    className?: string,
}

type Props = ButtonProps & ButtonHTMLAttributes<HTMLButtonElement>;

const Button: React.FC<Props> = (props) => {
    const {icon, label, position = 'after', className, ...others} = props;

    return (
        <button {...others} className={[classes.container, className].join(" ")}>
            {icon && position === 'before' && <i className="material-icons">{icon}</i>}
            <span>{label}</span>
            {icon && position === 'after' && <i className="material-icons">{icon}</i>}
        </button>
    );
};

export default Button;