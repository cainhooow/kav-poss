pub mod product_mapper;
pub mod role_mapper;
pub mod user_mapper;

pub trait Mapper<I, D> {
    fn to_domain(string: String) -> D;
    fn to_infra(domain: D) -> I;
}
