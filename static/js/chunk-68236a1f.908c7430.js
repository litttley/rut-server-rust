(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-68236a1f"],{"3b9d":function(t,e,a){"use strict";var s=a("f238"),n=a.n(s);n.a},ad42:function(t,e,a){"use strict";a.r(e);var s=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("div",{staticClass:"tag-page"},[a("div",{staticClass:"tag-side"},[a("h4",{staticClass:"sidetitle"},[t._v("Related Tags")]),t._l(t.relatedTags,function(e,s){return a("div",{key:s,staticClass:"sidebody"},[a("router-link",{attrs:{to:"/tag/"+e}},[t._v(t._s(e))])],1)})],2),a("div",{staticClass:"tagmeta"},[a("h4",[a("b",{staticStyle:{"font-size":"1.6em"}},[t._v(t._s(t.tname))]),a("el-button",{attrs:{type:"text"},on:{click:t.toEditTag}},[a("small",{staticStyle:{"font-size":"0.65em"}},[t._v("...Edit")])])],1),a("div",[a("div",{domProps:{innerHTML:t._s(t.detailIntro||"...")}}),t.less?a("el-button",{attrs:{type:"text",size:"mini"},on:{click:function(e){t.short=!1}}},[t._v("\n        ...More\n      ")]):t._e(),t.short?t._e():a("el-button",{attrs:{type:"text",size:"mini"},on:{click:function(e){t.short=!0}}},[t._v("\n        ...Less\n      ")])],1),a("div",{staticClass:"fobtn"},[a("el-button",{attrs:{size:"mini"},on:{click:t.starOrUnstarTag}},[t._v("\n        "+t._s("unstar"===t.starStatus?"Follow":"UnFollow")+"\n      ")]),a("br"),t.logo?a("img",{staticStyle:{"max-width":"65px","max-height":"65px","margin-top":"10px"},attrs:{src:t.logo,alt:"Logo",referrerPolicy:"no-referrer"}}):t._e()],1)]),a("div",{staticClass:"submenu"},[a("router-link",{attrs:{to:"/tag/"+t.tname+"/r"}},[t._v("List")])],1),a("div",{staticClass:"tag-view"},[a("router-view")],1),a("el-dialog",{attrs:{title:"Edit Tag Description",width:"640px",visible:t.show},on:{"update:visible":function(e){t.show=e}}},[a("v-form",{ref:"form",staticClass:"tag-form"},[a("v-text-field",{attrs:{label:"Tag Name",counter:20,rules:t.nameRule},model:{value:t.tname,callback:function(e){t.tname=e},expression:"tname"}}),a("v-textarea",{attrs:{label:"Logo Image Url",counter:120,rules:t.lenRule,rows:1,"auto-grow":""},model:{value:t.logo,callback:function(e){t.logo=e},expression:"logo"}}),a("v-text-field",{attrs:{label:"Parent Tag"},model:{value:t.parent,callback:function(e){t.parent=e},expression:"parent"}}),a("v-textarea",{attrs:{label:"Description Tag",counter:"",rows:5,"auto-grow":""},model:{value:t.intro,callback:function(e){t.intro=e},expression:"intro"}})],1),a("el-button",{attrs:{type:"success"},on:{click:t.onEditTag}},[t._v("Update")])],1)],1)},n=[],i=a("d722"),o=a("0a5a"),r=a("5ad4"),l=a("e6d6"),u={name:"tag-view",title:function(){return this.tname},data:function(){return{starStatus:"unstar",starCount:0,show:!1,short:!0,less:!0,tag:{},tname:"",parent:"",logo:"",intro:"",relatedTags:[],lenRule:[function(t){return t.length<=120||"Must be less than 120 characters"}],nameRule:[function(t){return!!t||"required"},function(t){return t.length<=20||"Must be less than 20 characters"}]}},computed:{detailIntro:function(){var t=Object(l["a"])(this.intro),e=128;return this.less=t.length>e&&this.short,this.less?Object(r["showLess"])(t,e):t}},methods:{loadTag:function(){var t=this,e=this.tname=this.$route.params.id;Object(i["o"])(e).then(function(e){t.setData(e.data.tag)})},setData:function(t){this.tag=t,this.tname=t.tname,this.logo=t.logo,this.intro=t.intro,this.starCount=t.satr_count},toEditTag:function(){Object(o["a"])()?this.show=!0:this.$message("Need to Log in")},onEditTag:function(t,e){var a=this;if(this.$refs.form.validate())if(Object(o["a"])()){var s={tname:this.tname.trim(),intro:this.intro.trim(),logo:this.logo.trim(),pname:this.parent.trim()};Object(i["D"])(this.tname,s).then(function(t){a.show=!1,a.setData(t.data.tag)})}else this.$message({showClose:!0,message:"Please Log in to Continue"}),this.$router.push({path:"/login",query:{redirect:this.$route.fullPath}});else console.log("Invalid")},checkStar:function(){var t=this;Object(o["a"])()?Object(i["e"])(this.tname).then(function(e){200===e.data.status&&(t.starStatus=e.data.message)}):this.starStatus="unstar"},starOrUnstarTag:function(){var t=this;Object(o["a"])()||(this.$message({message:"Please Login"}),this.$router.push({path:"/login",query:{redirect:this.$route.fullPath}})),"unstar"===this.starStatus?Object(i["y"])(this.tname,1).then(function(e){t.starStatus=e.data.message}):Object(i["y"])(this.tname,0).then(function(e){t.starStatus=e.data.message})}},watch:{"$route.params.id":"loadTag"},created:function(){this.loadTag(),this.checkStar()}},c=u,h=(a("3b9d"),a("2877")),d=Object(h["a"])(c,s,n,!1,null,"2dad6311",null);e["default"]=d.exports},f238:function(t,e,a){}}]);
//# sourceMappingURL=chunk-68236a1f.908c7430.js.map