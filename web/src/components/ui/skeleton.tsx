import type { FC, HTMLAttributes, ReactElement } from "react"

import { cn } from "../../lib/utils.ts"

type TSkeletonProps = HTMLAttributes<HTMLDivElement>

export const Skeleton: FC<TSkeletonProps> = ({
	className,
	...props
}: TSkeletonProps): ReactElement => (
	<div
		data-slot="skeleton"
		aria-hidden="true"
		className={cn("animate-pulse rounded-none bg-accent", className)}
		{...props}
	/>
)
