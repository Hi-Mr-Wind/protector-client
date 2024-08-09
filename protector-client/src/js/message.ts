import {ref} from "vue";

const input_message = ref("")
const user = ref({
    //用户ID
    userId:"",
    //用户名
    username: "张三",
    //头像地址
    path: "",
});

const message_list = ref(
    [
        {
            // 消息ID
            mesId:"",
            // 发送人昵称
            mesUser:"张三",
            //消息内容
            mes:"你好啊",
            //消息时间
            times:155516161321,
            //消息来源
            source:"1"
        },
        {
            // 消息ID
            mesId:"",
            // 发送人昵称
            mesUser:"李四",
            //消息内容
            mes:"非常好破千万IE发你啦收款登记哦去哪都去哪都对哦in钱啊山东潍坊南无阿死哪打磨器去哦我ID你哦啊是林东青蛙你懂阿萨",
            //消息时间
            times:155516161321,
            //消息来源
            source:"0"
        },
    ]
)
function getBackgroundColor(source:string):string{
    if (source=== "1"){
        return "#c6ffdb"
    }
    return "whitesmoke"
}

export {
    user,
    message_list,
    input_message,
    getBackgroundColor
}