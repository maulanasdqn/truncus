import { ChevronLeftIcon, ChevronRightIcon } from "lucide-react"
import type { FC, ReactElement } from "react"

import { Button } from "#/components/ui/button.tsx"

type TProps = {
	page: number
	pageCount: number
	onPageChange: (page: number) => void
}

const windowed = (page: number, pageCount: number): (number | "ellipsis")[] => {
	const set = new Set<number>()
	for (const candidate of [1, 2, pageCount - 1, pageCount, page - 1, page, page + 1]) {
		if (candidate >= 1 && candidate <= pageCount) set.add(candidate)
	}
	const sorted = [...set].sort((a, b) => a - b)
	const out: (number | "ellipsis")[] = []
	let prev = 0
	for (const value of sorted) {
		if (value - prev > 1) out.push("ellipsis")
		out.push(value)
		prev = value
	}
	return out
}

export const Pagination: FC<TProps> = ({
	page,
	pageCount,
	onPageChange,
}): ReactElement | null => {
	if (pageCount <= 1) return null
	return (
		<nav
			aria-label="Pagination"
			className="flex flex-wrap items-center justify-center gap-1"
		>
			<Button
				variant="ghost"
				size="sm"
				disabled={page <= 1}
				onClick={() => onPageChange(page - 1)}
			>
				<ChevronLeftIcon className="size-4" />
				Prev
			</Button>
			{windowed(page, pageCount).map((item, index) =>
				item === "ellipsis" ? (
					<span
						key={`ellipsis-${index}`}
						className="px-1.5 text-sm text-muted-foreground"
					>
						…
					</span>
				) : (
					<Button
						key={item}
						variant={item === page ? "default" : "ghost"}
						size="icon-sm"
						aria-current={item === page ? "page" : undefined}
						onClick={() => onPageChange(item)}
					>
						{item}
					</Button>
				),
			)}
			<Button
				variant="ghost"
				size="sm"
				disabled={page >= pageCount}
				onClick={() => onPageChange(page + 1)}
			>
				Next
				<ChevronRightIcon className="size-4" />
			</Button>
		</nav>
	)
}
