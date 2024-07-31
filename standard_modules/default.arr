ops,=,function,
    self,=,empty,
    f,=,function,self,other,
        return,NotImplemented,
    end,
    attr,self,ad,=,f,
    attr,self,sb,=,f,
    attr,self,ml,=,f,
    attr,self,dv,=,f,
    attr,self,pw,=,f,
    attr,self,md,=,f,
    attr,self,eq,=,f,
    attr,self,ne,=,f,
    attr,self,lt,=,f  ,#Low
    attr,self,le,=,f  ,#LowOrEqual
    attr,self,gt,=,f  ,#Big
    attr,self,ge,=,f  ,#BigOrEqual
    return,self,
end,

RationalNumber,=,namespace,
    new,=,function,numerator,denominator,
        self,=,empty,
        attr,self,nume,=,numerator,
        attr,self,deno,=,denominator,
        return,self,
    end,
end,