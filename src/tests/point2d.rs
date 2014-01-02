use linalg::point2d::Point2D;

#[test]
fn test_new()
{
    let p = Point2D::new(5, 5);

    assert_eq!(p.x, 5);
    assert_eq!(p.y, 5);
}