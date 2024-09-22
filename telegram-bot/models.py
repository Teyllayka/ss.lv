# models.py

from sqlalchemy import (
    Column, Integer, String, Float, Boolean, DateTime, ForeignKey, ARRAY, UniqueConstraint
)
from sqlalchemy.orm import declarative_base, relationship
from sqlalchemy.sql import func

Base = declarative_base()

class User(Base):
    __tablename__ = 'user'  # Ensure this matches SeaORM's table name

    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    created_at = Column(DateTime(timezone=True), server_default=func.now(), nullable=False)
    updated_at = Column(DateTime(timezone=True), onupdate=func.now(), nullable=False)
    avatar_url = Column(String, nullable=False, default="")
    name = Column(String, nullable=False)
    surname = Column(String, nullable=False)
    email = Column(String, unique=True, nullable=False)
    phone = Column(String, nullable=False)
    balance = Column(Float, nullable=False, default=0.0)
    password_hash = Column(String, nullable=False)
    is_admin = Column(Boolean, nullable=False, default=False)
    email_verified = Column(Boolean, nullable=False, default=False)
    telegram_id = Column(Integer, nullable=True)

    # Relationships
    adverts = relationship(
        "Advert",
        back_populates="user",
        foreign_keys='Advert.user_id'
    )
    favorites = relationship("Favorites", back_populates="user")
    reviews = relationship("Reviews", back_populates="user")


class Advert(Base):
    __tablename__ = 'advert'

    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    title = Column(String, nullable=False)
    description = Column(String, nullable=False)
    created_at = Column(DateTime(timezone=True), server_default=func.now(), nullable=False)
    updated_at = Column(DateTime(timezone=True), onupdate=func.now(), nullable=False)
    photo_url = Column(String, nullable=False)
    additional_photos = Column(ARRAY(String), nullable=False, default=[])
    available = Column(Boolean, nullable=False, default=True)
    price = Column(Float, nullable=False)
    location = Column(String, nullable=False)
    user_id = Column(Integer, ForeignKey('user.id'), nullable=False)
    category = Column(String, nullable=False)
    sold_to = Column(Integer, ForeignKey('user.id'), nullable=True)
    old_price = Column(Float, nullable=False, default=0.0)

    # Relationships
    user = relationship(
        "User",
        back_populates="adverts",
        foreign_keys=[user_id]
    )
    specifications = relationship("Specifications", back_populates="advert")
    favorites = relationship("Favorites", back_populates="advert")
    reviews = relationship("Reviews", back_populates="advert")


class Specifications(Base):
    __tablename__ = 'specifications'

    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    key = Column(String, nullable=False)
    value = Column(String, nullable=False)
    advert_id = Column(Integer, ForeignKey('advert.id', ondelete='CASCADE'), nullable=False)

    # Relationships
    advert = relationship("Advert", back_populates="specifications")


class Favorites(Base):
    __tablename__ = 'favorites'
    __table_args__ = (
        UniqueConstraint('user_id', 'advert_id', name='unique_user_advert'),
    )

    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    created_at = Column(DateTime(timezone=True), server_default=func.now(), nullable=False)
    user_id = Column(Integer, ForeignKey('user.id', ondelete='CASCADE'), nullable=False)
    advert_id = Column(Integer, ForeignKey('advert.id', ondelete='CASCADE'), nullable=False)

    # Relationships
    user = relationship("User", back_populates="favorites")
    advert = relationship("Advert", back_populates="favorites")


class Reviews(Base):
    __tablename__ = 'reviews'
    __table_args__ = (
        UniqueConstraint('advert_id', name='unique_advert_review'),
    )

    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    advert_id = Column(Integer, ForeignKey('advert.id', ondelete='CASCADE'), nullable=False, unique=True)
    user_id = Column(Integer, ForeignKey('user.id', ondelete='CASCADE'), nullable=False)
    created_at = Column(DateTime(timezone=True), server_default=func.now(), nullable=False)
    rating = Column(Integer, nullable=False)
    message = Column(String, nullable=False)

    # Relationships
    user = relationship("User", back_populates="reviews")
    advert = relationship("Advert", back_populates="reviews")
