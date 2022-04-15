SELECT id,
  SUM(CASE WHEN month="Jan" then revenue else null END) as Jan_Revenue,
  SUM(CASE WHEN month="Feb" then revenue else null END) as Feb_Revenue,
  SUM(CASE WHEN month="Mar" then revenue else null END) as Mar_Revenue,
  SUM(CASE WHEN month="Apr" then revenue else null END) as Apr_Revenue,
  SUM(CASE WHEN month="May" then revenue else null END) as May_Revenue,
  SUM(CASE WHEN month="Jun" then revenue else null END) as Jun_Revenue,
  SUM(CASE WHEN month="Jul" then revenue else null END) as Jul_Revenue,
  SUM(CASE WHEN month="Aug" then revenue else null END) as Aug_Revenue,
  SUM(CASE WHEN month="Sep" then revenue else null END) as Sep_Revenue,
  SUM(CASE WHEN month="Oct" then revenue else null END) as Oct_Revenue,
  SUM(CASE WHEN month="Nov" then revenue else null END) as Nov_Revenue,
  SUM(CASE WHEN month="Dec" then revenue else null END) as Dec_Revenue
  FROM Department GROUP BY id;
