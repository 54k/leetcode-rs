-- https://leetcode.com/problems/second-highest-salary/
select
    (
        select
            distinct salary
        from
            employee
        order by
            salary desc
        limit
            1 offset 1
    ) as SecondHighestSalary;

-- https://leetcode.com/problems/product-sales-analysis-i/description/
select
    p.product_name,
    s.year,
    s.price
from
    sales s
    left join product p on p.product_id = s.product_id
order by
    p.product_name;

-- https://leetcode.com/problems/product-sales-analysis-v/description/
select
    *
from
    (
        select
            p.user_id,
            sum(p.price) as spending
        from
            (
                select
                    s.user_id,
                    (p.price * s.quantity) as price
                from
                    sales s
                    left join product p on p.product_id = s.product_id
            ) p
        group by
            p.user_id
    ) q
order by
    q.spending desc,
    q.user_id asc;

-- https://leetcode.com/problems/nth-highest-salary/description/
CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT BEGIN
SET
    N = N - 1;

RETURN (
    # Write your MySQL query statement below.
    select
        *
    from
        (
            select
                distinct salary
            from
                Employee
            order by
                salary desc
            limit
                1 offset N
        ) as t
);

END