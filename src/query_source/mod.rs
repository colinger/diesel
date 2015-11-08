// mod filter;
mod joins;

use expression::{Expression, SelectableExpression, NonAggregate};
use query_builder::*;
// pub use self::filter::FilteredQuerySource;
pub use self::joins::{InnerJoinSource, LeftOuterJoinSource};
use types::{FromSqlRow, NativeSqlType};

pub use self::joins::JoinTo;

pub trait Queriable<ST: NativeSqlType> {
    type Row: FromSqlRow<ST>;

    fn build(row: Self::Row) -> Self;
}

pub trait QuerySource: Sized {
    fn from_clause<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult;
    // fn where_clause<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult;

//     fn filter<T>(self, predicate: T) -> FilteredQuerySource<Self, T> where
//         T: SelectableExpression<Self, types::Bool>,
//     {
//         FilteredQuerySource::new(self, predicate)
//     }
}

pub trait Column {
    type Table: Table;
    type SqlType: NativeSqlType;

    fn name(&self) -> String;

    fn qualified_name(&self) -> String;
}

impl<C: Column> Expression for C {
    type SqlType = <Self as Column>::SqlType;

    fn to_sql<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult {
        out.push_sql(&self.qualified_name());
        Ok(())
    }
}

impl<C: Column> SelectableExpression<C::Table> for C {
}

impl<C: Column> NonAggregate for C {
}

pub trait Table: QuerySource + AsQuery + Sized {
    type PrimaryKey: Column<Table=Self>;
    type Star: Column<Table=Self>;

    fn name(&self) -> &str;
    fn primary_key(&self) -> Self::PrimaryKey;
    fn star(&self) -> Self::Star;

    fn inner_join<T>(self, other: T) -> InnerJoinSource<Self, T> where
        T: Table,
        Self: JoinTo<T>,
    {
        InnerJoinSource::new(self, other)
    }

    fn left_outer_join<T>(self, other: T) -> LeftOuterJoinSource<Self, T> where
        T: Table,
        Self: JoinTo<T>,
    {
        LeftOuterJoinSource::new(self, other)
    }
}