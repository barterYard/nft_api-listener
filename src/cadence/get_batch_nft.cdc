import NonFungibleToken from "./contracts/lib/NonFungibleToken.cdc"
import MetadataViews from "./contracts/lib/MetadataViews.cdc"

pub struct NFTDisplay {
    pub let nft: MetadataViews.Display
    pub let traits: [MetadataViews.Trait]?
    pub let tokenID: UInt64

    init(tokenID: UInt64,display: MetadataViews.Display, traits: MetadataViews.Traits?) {
        self.tokenID = tokenID
        self.nft = display
        self.traits = traits?.traits
    }
}

pub fun main(address: Address, publicIdentifier: String, nftIds: [UInt64]): [NFTDisplay] {
    let userCollection = getAccount(address).getCapability < &{MetadataViews.ResolverCollection} > (PublicPath(identifier: publicIdentifier)!).borrow() ?? panic("could not resolve collection")
    var res: [NFTDisplay] = []
    for nftId in nftIds {
      let nft = userCollection.borrowViewResolver(id: nftId)!
      let view = nft.resolveView(Type < MetadataViews.Display > ())! as! MetadataViews.Display
      var traits: MetadataViews.Traits? = nil
      if let traitsView = nft.resolveView(Type < MetadataViews.Traits > ()) {
          traits = traitsView as! MetadataViews.Traits
      }
      res.append(NFTDisplay(tokenID: nftId, display: view, traits: traits))
    }

    return res
}
 