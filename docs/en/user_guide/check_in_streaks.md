# Check-in Streaks & Statistics

The Check-in Streaks feature provides detailed check-in history, streak statistics, and visual analysis to help you track your check-in habits and earnings.

![Check-in Streaks Interface](../../assets/app-streak-en.png)

## Feature Overview

- 📊 **Streak Statistics** - Current streak days, longest streak record
- 📅 **Calendar View** - Visual display of monthly check-in status
- 📈 **Trend Analysis** - 30-day check-in trend chart
- 💰 **Earnings Tracking** - Balance increment for each check-in
- 🎯 **Per-Account View** - View individual account or all accounts

## Page Navigation

1. Click **Check-in Streaks** in the sidebar to enter the page
2. Use the account selector in the top right to switch views:
   - **All Accounts** - View overview of all accounts
   - **Individual Account** - View detailed records for a specific account

## All Accounts View

### Overview Cards

Display all accounts grouped by service provider:

| Display Content | Description |
|----------------|-------------|
| Account Name | Display name of the account |
| Last Check-in | Most recent check-in date |
| Current Streak | Current consecutive check-in days |
| Longest Streak | Historical longest consecutive days |
| Total Days | Cumulative total check-in days |

### Actions

- **Click Card** - Switch to detailed view for that account

## Individual Account View

After selecting a specific account, you can view detailed check-in records and statistics.

### Account Overview Panel

The left panel displays key statistics for the account:

#### Basic Information
- Account name
- Service provider
- Last check-in date

#### Streak Statistics
- **Current Streak** 🟠 - Current consecutive check-in days
- **Longest Streak** 🟡 - Historical longest consecutive days
- **Total Days** 🔵 - Cumulative total check-in days

#### Monthly Statistics
- **Check-in Days** - Days checked in this month / Total days this month
- **Check-in Rate** - Percentage of check-in rate this month
- **Earnings Increment** - Total check-in earnings this month (USD)

### Check-in Calendar

The right calendar displays monthly check-in status:

#### Date Markers
- 🟢 **Green** - Checked in (shows earnings increment)
- ⚪ **Gray** - Not checked in
- 🔵 **Blue Border** - Today

#### Calendar Operations
- Click **Previous/Next Month** buttons to switch months
- Click on a date to view details for that day

### Check-in Trend Chart

Displays check-in trend for the past 30 days:

- **X-axis** - Date
- **Y-axis** - Earnings increment (USD)
- **Data Points** - Daily check-in earnings
- **Trend Line** - Visual display of earnings changes

## View Check-in Details

Click on a date in the calendar to open a details dialog:

| Field | Description |
|-------|-------------|
| Date | Check-in date |
| Status | Check-in status (success/failed/not checked in) |
| Balance Increment | Balance increase from this check-in |
| Current Balance | Balance after check-in |
| Check-in Time | Specific time of check-in |
| Notes | Check-in result message or error information |

## Streak Rules

### Streak Calculation Rules

1. **Start Streak** - After successful check-in, streak count starts at 1
2. **Maintain Streak** - Check in every day, streak days +1
3. **Break Streak** - Miss a day, streak days reset to zero
4. **Longest Record** - Automatically tracks historical longest streak days

### Streak Example

```
Date:    1st  2nd  3rd  4th  5th  6th  7th
Checked: ✓    ✓    ✓    ✗    ✓    ✓    ✓
Streak:  1    2    3    0    1    2    3
Longest: 1    2    3    3    3    3    3
```

## Check-in Rate Calculation

Check-in Rate = (Days checked in this month / Total days this month) × 100%

**Example**:
- 30 days this month
- 25 days checked in
- Check-in Rate = (25 / 30) × 100% = 83.3%

## Earnings Tracking

### Single Check-in Earnings

Each check-in's earnings are displayed below the calendar date:

```
15
+$0.50
```

### Monthly Earnings

Monthly statistics show total earnings from all check-ins this month:

```
Earnings Increment: $12.50
```

### Trend Analysis

Through the trend chart you can observe:
- Daily earnings changes
- Earnings differences between providers
- Earnings stability

## Usage Tips

### Build Check-in Habits

1. **Set Auto Check-in**
   - Edit account, enable auto check-in
   - Set appropriate check-in time (e.g., 8:00 AM)

2. **Regular Monitoring**
   - Check the streaks page weekly
   - Ensure no missed check-ins
   - Monitor streak day changes

3. **Track Goals**
   - Set consecutive check-in goals (e.g., 30 days, 90 days)
   - Use calendar to track progress

### Optimize Earnings

1. **Analyze Trends**
   - Observe which accounts have higher earnings
   - Focus on earnings change trends

2. **Handle Failures Promptly**
   - View details for failed dates
   - Adjust configuration based on error messages

3. **Multi-Account Management**
   - Use all accounts view to monitor overall situation
   - View individual accounts that need attention

## Data Statistics

### Available Metrics

| Metric | Description | Location |
|--------|-------------|----------|
| Current Streak | Current consecutive check-in days | Account overview |
| Longest Streak | Historical longest consecutive days | Account overview |
| Total Days | Cumulative total check-in days | Account overview |
| Monthly Check-ins | Check-in days this month | Monthly stats |
| Check-in Rate | Check-in rate this month | Monthly stats |
| Monthly Earnings | Total earnings this month | Monthly stats |

### Cross-Month Streaks

Streak calculation rules across months:
- December 31 check-in + January 1 check-in = Streak maintained
- Missing any day = Streak broken

## Common Questions

### Q: Why did my streak reset to zero?

**A**: Possible reasons:
- Forgot to check in on a day (manual or automatic)
- Check-in failed (view details for failed date)
- App wasn't running, causing auto check-in to not execute

### Q: How long are check-in records kept?

**A**: Check-in records are permanently saved in the local database and won't be automatically cleared.

### Q: Can I modify historical check-in records?

**A**: Manual modification of historical records is not currently supported. All records are automatically generated by the system.

### Q: Why do some dates not have earnings increments?

**A**: Possible reasons:
- Check-in failed
- Provider didn't provide earnings that day
- Failed to get balance information

### Q: Why does the trend chart only show 30 days?

**A**: To keep the chart clear and readable, it defaults to showing the most recent 30 days. Custom time ranges may be supported in future versions.

### Q: Can I export check-in records?

**A**: Export functionality is not currently supported, planned for future versions.

## Tips & Suggestions

### Improve Check-in Success Rate

1. **Keep App Running**
   - Auto check-in requires app to be running
   - Recommend setting startup on boot

2. **Check Network**
   - Ensure network is normal during check-in
   - Check proxy settings

3. **Update Credentials**
   - Update cookies promptly when expired
   - Regularly check account status

### Make Full Use of Statistics

1. **Set Goals**
   - Track consecutive check-in days
   - Set monthly earnings goals

2. **Comparative Analysis**
   - Compare performance of different accounts
   - Analyze reasons for earnings changes

3. **Optimize Strategy**
   - Adjust check-in time based on statistics
   - Focus on high-value accounts

## Related Documentation

- [Check-in Operations](./check_in_operations.md) - Learn how to perform check-ins
- [Account Management](./account_management.md) - Manage accounts and auto check-in
- [Balance Tracking](./balance_tracking.md) - View balance changes
