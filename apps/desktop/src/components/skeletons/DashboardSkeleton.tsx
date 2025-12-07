import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"

export function DashboardSkeleton() {
  return (
    <div className="space-y-8">
      <div className="flex items-end justify-between">
        <div className="space-y-2">
          <Skeleton className="h-8 w-48" />
          <Skeleton className="h-4 w-96" />
        </div>
      </div>

      {/* Bento Grid Overview Skeleton */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {/* Main Balance Card Skeleton */}
        <div className="md:col-span-2">
          <Card className="h-full">
            <CardHeader className="pb-2">
              <Skeleton className="h-4 w-32" />
            </CardHeader>
            <CardContent>
              <Skeleton className="h-10 w-48 mb-2" />
              <Skeleton className="h-4 w-24" />
            </CardContent>
          </Card>
        </div>

        {/* Income & Consumption Stack Skeleton */}
        <div className="space-y-4">
          <Card className="bg-muted/30 border-none shadow-none">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <Skeleton className="h-4 w-24" />
              <Skeleton className="h-4 w-4 rounded-full" />
            </CardHeader>
            <CardContent>
              <Skeleton className="h-8 w-32" />
            </CardContent>
          </Card>
          <Card className="bg-muted/30 border-none shadow-none">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <Skeleton className="h-4 w-32" />
              <Skeleton className="h-4 w-4 rounded-full" />
            </CardHeader>
            <CardContent>
              <Skeleton className="h-8 w-32" />
            </CardContent>
          </Card>
        </div>

        {/* Accounts Status Skeleton */}
        <div>
          <Card className="h-full flex flex-col justify-center bg-muted/30 border-none shadow-none">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <Skeleton className="h-4 w-28" />
              <Skeleton className="h-4 w-4 rounded-full" />
            </CardHeader>
            <CardContent>
              <Skeleton className="h-9 w-16 mb-2" />
              <Skeleton className="h-4 w-24" />
            </CardContent>
          </Card>
        </div>
      </div>

      {/* Provider Breakdown Skeleton */}
      <div>
        <Skeleton className="h-6 w-48 mb-4" />
        <div className="grid gap-6">
          {[1, 2].map((i) => (
            <Card key={i} className="overflow-hidden border-none shadow-sm bg-card/50">
              <CardContent className="p-0">
                <div className="p-6 bg-muted/20 border-b border-border/50 flex items-center justify-between">
                  <div className="space-y-2">
                    <Skeleton className="h-6 w-32" />
                    <Skeleton className="h-4 w-24" />
                  </div>
                  <div className="text-right space-y-1">
                    <Skeleton className="h-3 w-24 ml-auto" />
                    <Skeleton className="h-6 w-32" />
                  </div>
                </div>
                <div className="p-6 space-y-4">
                  <Skeleton className="h-24 w-full rounded-xl" />
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  )
}
