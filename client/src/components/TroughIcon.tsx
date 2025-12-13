interface TroughIconProps {
  className?: string;
}

export function TroughIcon({ className = "h-6 w-6" }: TroughIconProps) {
  return (
    <svg
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
    >
      <path d="M3 8 L5 18 L19 18 L21 8" />
      <path d="M2 8 L22 8" />
      <path d="M7 8 L7 5" />
      <path d="M17 8 L17 5" />
    </svg>
  );
}
