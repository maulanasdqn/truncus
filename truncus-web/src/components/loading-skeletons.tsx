import type { CSSProperties, FC, ReactElement } from "react"

import { Card, CardContent } from "#/components/ui/card.tsx"
import { Skeleton } from "#/components/ui/skeleton.tsx"

const range = (length: number): readonly number[] =>
	Array.from({ length }, (_, index) => index)

const GRID_COLUMNS: Record<3 | 4, string> = {
	3: "grid gap-4 lg:grid-cols-3",
	4: "grid gap-4 lg:grid-cols-4",
}

type TSkeletonStatGridProps = {
	cards?: number
	columns?: 3 | 4
}

export const SkeletonStatGrid: FC<TSkeletonStatGridProps> = ({
	cards = 4,
	columns = 4,
}): ReactElement => (
	<ul aria-busy="true" className={GRID_COLUMNS[columns]}>
		{range(cards).map((index) => (
			<li key={index}>
				<Card className="h-full">
					<CardContent className="flex flex-col gap-3">
						<Skeleton className="h-3 w-24" />
						<Skeleton className="h-9 w-20" />
						<Skeleton className="h-3 w-32" />
					</CardContent>
				</Card>
			</li>
		))}
	</ul>
)

type TSkeletonTableProps = {
	rows?: number
	columns?: number
}

const rowStyle = (columns: number): CSSProperties => ({
	gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))`,
})

export const SkeletonTable: FC<TSkeletonTableProps> = ({
	rows = 5,
	columns = 5,
}): ReactElement => (
	<Card aria-busy="true">
		<CardContent className="flex flex-col gap-5">
			<div className="grid gap-4" style={rowStyle(columns)}>
				{range(columns).map((index) => (
					<Skeleton key={index} className="h-3 w-full max-w-24" />
				))}
			</div>
			{range(rows).map((row) => (
				<div key={row} className="grid gap-4" style={rowStyle(columns)}>
					{range(columns).map((column) => (
						<Skeleton key={column} className="h-4 w-full" />
					))}
				</div>
			))}
		</CardContent>
	</Card>
)

export const SkeletonLines: FC<{ lines?: number }> = ({
	lines = 6,
}): ReactElement => (
	<div aria-busy="true" className="flex flex-col gap-3">
		{range(lines).map((index) => (
			<Skeleton
				key={index}
				className="h-4"
				style={{ width: `${70 + ((index * 37) % 30)}%` }}
			/>
		))}
	</div>
)
