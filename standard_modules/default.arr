ops,=,function,
   ,self,=,empty
   ,f,=,function,self,other
       ,return,NotImplemented
   ,end
   ,attr,self,ad,=,f
   ,attr,self,sb,=,f
   ,attr,self,ml,=,f
   ,attr,self,dv,=,f
   ,attr,self,pw,=,f
   ,attr,self,md,=,f
   ,attr,self,eq,=,f
   ,attr,self,ne,=,f
   ,attr,self,lt,=,f  ,#Low
   ,attr,self,le,=,f  ,#LowOrEqual
   ,attr,self,gt,=,f  ,#Big
   ,attr,self,ge,=,f  ,#BigOrEqual
   ,return,self,
end

Boolean,=,namespace
   ,new,=,function,x
       ,attr,x,bool,to_bool
       ,return,call,ref,tmp,1
   ,end
end

Byte,=,namespace
   ,new,=,function,on,tw,th,fo,fi,si,se,ei
       ,self,=,empty
       ,attr,self,one,=,on
       ,attr,self,two,=,tw
       ,attr,self,three,=,th
       ,attr,self,four,=,fo
       ,attr,self,five,=,fi
       ,attr,self,six,=,si
       ,attr,self,six,=,si
       ,attr,self,seven,=,se
       ,attr,self,eight,=,ei
       ,return,self
   ,end
end

List,=,namespace
   ,new,=,function,arg
       ,tmpL,=,empty
       ,index,=,0
       ,next_func,=,attr,arg,next
       ,loop
           ,ref,tmpL,index,=,call,next_func
           ,index,+=,1
           ,switch
               ,case,index,==,attr,arg,len
                   ,break
               ,end
           ,end
       ,end
       ,return,tmpL
   ,end
end

Char,=,namespace
   ,new,=,function,unicode_bytes
       ,self,=,empty
       ,attr,self,codes,=,unicode_bytes
       ,return,self
   ,end
end

String,=,namespace
   ,new,=,function,chars
       ,self,=,empty
       ,attr,self,chars,=,chars
       ,return,self
   ,end
end

Int,=,namespace
   ,new,=,function,x
       ,self,=,empty
       ,to_int_func,=,attr,x,to_int
       ,attr,self,val,=,call,to_int_func
       ,return,self
   ,end
end

RationalNumber,=,namespace
   ,new,=,function,numerator,denominator
       ,self,=,empty
       ,attr,self,nume,=,numerator
       ,attr,self,deno,=,denominator
       ,return,self
   ,end
end