pub fn should_receive_discount(
    purchases_last_month: usize,
    purchases_month_before: usize,
    avg_purchase_price: f32,
) -> bool {
    let purchased_more_than_10_products_previous_month =
        purchases_month_before > 10;
    let avg_price_exceeds_10_dollars = avg_purchase_price > 10.0;
    let purchases_last_month_is_smaller_than_previous_month =
        purchases_last_month < purchases_month_before;

    (purchased_more_than_10_products_previous_month
        && purchases_last_month_is_smaller_than_previous_month
        && avg_price_exceeds_10_dollars)
        || (avg_price_exceeds_10_dollars
            && purchased_more_than_10_products_previous_month)
}
