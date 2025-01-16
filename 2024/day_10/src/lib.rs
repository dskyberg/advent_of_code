use thiserror::Error;

pub const TRAIL_HEAD_HEIGHT: u8 = 0;
pub const TRAIL_MAX_HEIGHT: u8 = 9;

#[derive(Debug, Error)]
pub enum Day10Error {
    #[error("DAG is incomplete")]
    IncompleteDAG,
}

pub static INPUT: &str = r#"981050112210121034565432321032103321234567567
872341201014434921878891434549012310432198678
765434318123567890989760023678543986501021549
545985329854356792105654116707621677893430932
456576416763447883498743209811030543082567801
327676509823430976541050124322345892121078998
218489678016521050132011267401276734289010867
109345908987210763249121278569889825678123459
543237817876345854358230989078778910107645678
654106726541096981267345678123498723216530501
761235430432387870301076521014567234345321432
890144321001456765432189410019010121237876501
781055697878921212345674321928723210345989432
218966788965430101254789345839654234326710101
109875692100123270163231236743010165019823321
018754103018786789872100109852123278956754430
325673214109695698987212121961094109849869543
234981005234534321210103030878785603732778672
189012346943413210321234549879676712011001981
076321057892100110450321676566543893425652760
325401069787012026565410789457012798534743854
410512178976543237656998898308932687649887923
567893467689854148997867891219801456308996310
234554992501763056788950750304762343210105409
147667881432612234563441065403451012308712358
018589670109803189612532674312332305419654347
189478521210789078703675689100123498569323256
789767432345650876589986789210096567878012100
679854343217821987676521654323487458901223321
565943034506934910505430598714510329874341234
450012129645445823412014567609621210965210585
321101518789336798703123455418760145670195696
234965400689221209654328954303210234987784787
105874321678100118765017763214301001056653610
876701234549010329870126894505212892347542123
965891098238321234561235214676703083498230036
874782347107630145490344305987894176580121545
923690456016549854987432234589765237891234694
012541219897896768876501105676894396790346787
001432108766541089004321212988787781687656698
123498789655432376113410145679695670521044567
876567610141045645223567034034544321430123698
965458901232038764344038123129632487654310432
012307676540129854965129876038701898943216581
103212789432100123876434565345612781012107890
"#;
