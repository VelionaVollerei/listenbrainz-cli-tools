use crate::models::data::listenbrainz::listen::Listen;

pub struct ListenRemapper {
    listen: Vec<Listen>
}

impl ListenRemapper {
    // fn prompt_remmap(&self, listen: &Listen) {
    //     remapped_msids.push(listen.get_messybrain_data().msid().clone());

    //     println!();
    //     println!("{:#?}", listen.get_messybrain_data());
    //     println!();

    //     if !self.ask_remap_action(listen.as_ref()).await {
    //         return;
    //     }
    // }
}