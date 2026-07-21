import type { ReactElement, ReactNode } from "react"

import { cn } from "../../lib/utils.ts"

type TEmptyStateProps = {
	message: string
	title?: string
	icon?: ReactNode
	className?: string
}

const EmptyState = ({
	message,
	title,
	icon,
	className,
}: TEmptyStateProps): ReactElement => (
	<output
		data-slot="empty-state"
		className={cn(
			"flex flex-col items-center justify-center gap-2 p-8 text-center text-muted-foreground [&>svg]:size-8 [&>svg]:opacity-60",
			className,
		)}
	>
		{icon}
		{title && <p className="font-medium text-foreground">{title}</p>}
		<p className="text-sm">{message}</p>
	</output>
)

export { EmptyState }
export type { TEmptyStateProps }
