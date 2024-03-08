pub mod guild_repository;
pub mod times_repository;

pub use guild_repository::GuildRepository;
pub use times_repository::TimesRepository;

#[derive(Debug)]
pub struct Repository<G, T>
where
    G: GuildRepository,
    T: TimesRepository,
{
    pub guild_repository: G,
    pub times_repository: T,
}
