CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY,
  reputation INT NOT NULL
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS coupons (
  id UUID PRIMARY KEY,
  creator_id UUID NOT NULL,
  code TEXT NOT NULL,
  description TEXT NOT NULL,
  expiry TIMESTAMP WITH TIME ZONE,
  domain TEXT NOT NULL,
  score INT NOT NULL DEFAULT 1,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()

  FOREIGN KEY (creator_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_coupons_domain ON coupons (domain);

CREATE TABLE IF NOT EXISTS votes (
  id UUID PRIMARY KEY,
  coupon_id UUID NOT NULL,
  voter_id UUID NOT NULL,
  vote_type BOOLEAN NOT NULL, -- TRUE for upvote, FALSE for downvote
  UNIQUE (coupon_id, voter_id) -- Each user can vote once per coupon

  FOREIGN KEY (coupon_id) REFERENCES coupons (id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (voter_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_votes_coupon_id ON votes (coupon_id);
