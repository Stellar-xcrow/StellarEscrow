import React from 'react';
import './Button.css';

/**
 * Button — base interactive element used throughout StellarEscrow.
 *
 * Variants: primary | secondary | danger | success
 * Sizes:    sm | md | lg
 *
 * Shows a spinner and sets aria-busy when `loading` is true.
 * Forwards refs so it can be composed with tooltip/popover libraries.
 */
export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'danger' | 'success';
  size?: 'sm' | 'md' | 'lg';
  loading?: boolean;
  disabled?: boolean;
}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ variant = 'primary', size = 'md', loading = false, disabled = false, children, ...props }, ref) => {
    return (
      <button
        ref={ref}
        className={`btn btn-${variant} btn-${size}`}
        disabled={disabled || loading}
        aria-busy={loading}
        {...props}
      >
        {loading && <span className="btn-spinner" aria-hidden="true" />}
        {children}
      </button>
    );
  }
);

Button.displayName = 'Button';
