import NFTStorefrontV2 from 0x4eb8a10cb9f87357
import NFTStorefront from 0x4eb8a10cb9f87357
import NonFungibleToken from 0x1d7e57aa55817448

pub struct Result {
  pub let price: UFix64

  pub let storefrontID: UInt64
        /// Whether this listing has been purchased or not.
  pub let purchased: Bool
  /// The Type of the NonFungibleToken.NFT that is being listed.
  pub let nftType: Type
  /// The ID of the NFT within that type.
  pub let nftID: UInt64
  init( price: UFix64,
        storefrontID: UInt64,
        purchased: Bool,
        nftType: Type,
        nftID: UInt64) {
          self.price = price

          self.storefrontID = storefrontID
          self.purchased = purchased
          self.nftType = nftType
          self.nftID = nftID
  }
}
pub fun main(storefrontAddress:Address): {String: [Result?]} {
  let cap = getAccount(storefrontAddress).getCapability<&{NFTStorefront.StorefrontPublic}>(NFTStorefront.StorefrontPublicPath).borrow()
  let res: {String: [Result?]} = {}
  if let ids = cap?.getListingIDs() {
    for id in ids {
      if let listing = cap?.borrowListing(listingResourceID: id) {
        if let details = listing?.getDetails() {

          if res[details.nftType.identifier] == nil {
            res[details.nftType.identifier] = []
          }

            res[details.nftType.identifier]!.append(Result(
            price: details.salePrice,
            storefrontID:details.storefrontID,
            purchased:details.purchased,
            nftType:details.nftType,
            nftID:details.nftID ))

        }
      }
    }
  }
  return res
}
